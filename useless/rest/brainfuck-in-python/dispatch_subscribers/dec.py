from ignore_incoming_arguments import ignore
import dispatcher
import storage


dispatcher.register('-', ignore(lambda: storage.CurrentCell.set(storage.CurrentCell.get() - 1)))
