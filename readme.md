# mmm

midi mapping manager

## what does it do

i thought it would be fun to make a midi patchbay entirely in a cli. so that's what this is.

mmm consists of a map of "virtual processors" linked together. any processor (barring those send midi out of the application) can have zero to infinite outputs. any processor (barring those which take midi from outside the application) can have any number of inputs. all processors have a numeric id and a string name.

currently there are 3 processors:

### input
connects to and takes midi from an external port. takes 1 argument (an port index) on initialisation

### output
provides a virtual port that other applications can connect to

### channelfilter
filters out messages on all but one channel. system global and realtime messages are passed through. takes 1 argument (a channel) on initialisation

## how to use

mmm uses an interactive shell as an interface. valid commands are:

### exit
exits the application by means of a panic (i'll fix it later)

### list
lists all processors currently in existence in the format `ID: DISPLAYNAME`. note that the display name is not the same as the internal name used when referring to a processor.

### ls
alias for list

### rename ID_OR_NAME NEW_NAME
rename a processor. processors can be referred to by numeric id or name.

### connect SRC DEST
connect two processors together. processors can be referred to by numeric id or name.

### disconnect SRC DEST
disconnect two processors. processors can be referred to by numeric id or name.

### cfg NAME_OR_ID \[COMMAND...\] (not yet implemented)
send a command to a processor.

### init TYPE NAME \[ARGS...\]
create a new processor.

### new
alias for init

### remove (not yet implemented)
remove a processor.

### inputs
list all external ports available for connection.

### outputs ID_OR_NAME
list all outputs of a given processor.

## other bits

any string value can be referred to by a shortened name so long as it is unambiguous. for example, `e` will work in place of `exit`, but `c` will not, because it could mean connect or cfg. the same is true of processor names and processor types.

## what is gen_consts.py for

i needed to generate a bunch of boring constants. const fns weren't enough, macros weren't quite suited, so i did the bad thing and used python.
