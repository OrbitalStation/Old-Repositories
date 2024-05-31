import socket, audioop, grpc, telebot, time, wave
from tinkoff.cloud.stt.v1 import stt_pb2_grpc, stt_pb2
from tinkoff.cloud.tts.v1 import tts_pb2_grpc, tts_pb2
from auth import authorization_metadata
from dataset import check
from config import *

# Constants

HOST = ''
PORT = 10000

GREETING = 'Здравствуйте, меня зовут Никита, я - автоответчик. Произнесите текст, по завершении вызова он будет отправлен хозяину.'

SAY_PACKET_SZ = 320
HEAD_SZ = 3

DELAY = 0.02

TELEGRAM_CHAT_ID = 1756668350
PHONE_NUMBER = ''


# Globals

dialog = bytes()

def build_request():
    request = stt_pb2.RecognizeRequest()
    request.audio.content = b
    request.config.encoding = stt_pb2.AudioEncoding.LINEAR16
    request.config.sample_rate_hertz = 8000
    request.config.num_channels = 1
    return request


def send_to_master():
    global dialog

    bot = telebot.TeleBot(TOKEN)

    msg = 'BOT:         ' + GREETING + '\n'
    spam = False

    for result in response.results:
        for alternative in result.alternatives:
            if alternative.transcript != '':
                spam = check(alternative.transcript)
                msg += 'CALLER:   ' + alternative.transcript.capitalize() + '.\n'
    # noinspection PyBroadException
    try:
        bot.send_message(TELEGRAM_CHAT_ID, '------------------------------------------------------------------------------------------------')
        bot.send_message(TELEGRAM_CHAT_ID, 'Call to your phone occurred.\nThere\'s a dialog between BOT and CALLER in text and audio formats.')
        bot.send_message(TELEGRAM_CHAT_ID, 'Text:')
        bot.send_message(TELEGRAM_CHAT_ID, msg)
        bot.send_message(TELEGRAM_CHAT_ID, 'Audio:')

        with wave.open('dialog.wav', 'w') as f:
            f.setframerate(8000)
            f.setnchannels(1)
            f.setsampwidth(2)
            f.writeframes(dialog)
        with open('dialog.wav', 'rb') as f:
            bot.send_audio(TELEGRAM_CHAT_ID, f.read())

        if spam:
            # TODO: Add to blacklist
            bot.send_message(TELEGRAM_CHAT_ID, f'As a result, CALLER is a spammer, so number "{PHONE_NUMBER}" is now in blacklist.\nYou can undo it at any moment at your phone app.')
    except Exception: pass


# noinspection PyShadowingNames
def say(text):
    global dialog

    # noinspection PyShadowingNames
    def build_request():
        return tts_pb2.SynthesizeSpeechRequest(
            input=tts_pb2.SynthesisInput(
                text=text
            ),
            audio_config=tts_pb2.AudioConfig(
                audio_encoding=tts_pb2.LINEAR16,
                sample_rate_hertz=48000,
            ),
        )
    stub = tts_pb2_grpc.TextToSpeechStub(grpc.secure_channel("tts.tinkoff.ru:443", grpc.ssl_channel_credentials()))
    request = build_request()
    metadata = authorization_metadata(API, SECRET, "tinkoff.cloud.tts")
    responses = stub.StreamingSynthesize(request, metadata=metadata)

    head = bytes([0x10, 0x01, 0x40])

    state = None
    for r in responses:
        data = r.audio_chunk
        data, state = audioop.ratecv(data, 2, 1, 48000, 8000, state)

        while len(data) >= SAY_PACKET_SZ:
            dialog += data[:SAY_PACKET_SZ]
            conn.send(head + data[:SAY_PACKET_SZ])
            time.sleep(DELAY)
            data = data[SAY_PACKET_SZ:]
            conn.recv(SAY_PACKET_SZ + HEAD_SZ)
        if len(data) > 0:
            dialog += data[:SAY_PACKET_SZ]
            conn.send(bytes([0x10, 0x00, len(data)]) + data)
            time.sleep(DELAY)
            conn.recv(HEAD_SZ + len(data))


while True:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind((HOST, PORT))
        s.listen(1)
        conn, addr = s.accept()
        with conn:
            print(f'Connected by {addr}.')
            b = bytes()
            meet = True
            while True:
                head = conn.recv(HEAD_SZ)
                if not head: break
                length = (int(head[1]) * 0x100) + (int(head[2]))
                buf = conn.recv(length)
                if head[0] != 0x10: continue
                buf = audioop.ulaw2lin(buf, 2)
                b += buf
                dialog += buf
                if meet:
                    meet = False
                    say(GREETING)
            stub = stt_pb2_grpc.SpeechToTextStub(grpc.secure_channel("stt.tinkoff.ru:443", grpc.ssl_channel_credentials()))
            metadata = authorization_metadata(API, SECRET, "tinkoff.cloud.stt")
            response = stub.Recognize(build_request(), metadata=metadata)
            send_to_master()
            print('Connection finished.')
