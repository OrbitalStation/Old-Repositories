from ignore_incoming_arguments import ignore
import dispatcher
import storage


dispatcher.register('>', ignore(storage.Cursor.forward))
