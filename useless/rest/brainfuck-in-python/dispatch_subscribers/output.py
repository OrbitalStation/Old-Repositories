from ignore_incoming_arguments import ignore
import dispatcher
import storage
import ui


dispatcher.register('.', ignore(lambda: ui.putchar(chr(storage.CurrentCell.get()))))
