from database import db
from commands import handle_unknown_command
from e_mail.send import send
from storage.upload_from_message import upload_from_message, MaxFilenameTagWatcher
from properties import const


_MAX_FILENAME_TAG_WATCHER = MaxFilenameTagWatcher()


def listener(cb):
    def inner(bot, message):
        if message.forward_from or message.forward_from_chat or message.forward_sender_name:
            cb(bot, message)
        else:
            if message.content_type == "text":
                message.text = message.text.strip()
                if message.text.startswith("/"):
                    handle_unknown_command(bot, message)
                    return
            bot.send_message(message.from_user.id, const("botNotForwardedMessageLis"))
    return inner


def attachment_listener():
    @listener
    def inner(bot, message):
        global _MAX_FILENAME_TAG_WATCHER

        if (file_url := upload_from_message(bot, message, _MAX_FILENAME_TAG_WATCHER)) is None:
            return
        if send(bot, message, db.fetch_user(message.from_user.id).email, message.caption if message.caption else "",
                const("botAttachmentUploadedToCloudStorage") + ' ' + file_url, message.caption_entities):
            bot.send_message(message.from_user.id, const("botMessageSentToEmailLis"))
    return inner
