from ignore_incoming_arguments import ignore
import dispatcher
import storage
import ui


dispatcher.register(',', ignore(lambda: storage.CurrentCell.set(ord(ui.getchar()))))
