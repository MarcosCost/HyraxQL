# App State

Appstate will send messages when data is updated, the core app fails, etc... 
Message codes are send in the channel so here is the documented use for each:
- 0 : Everything is fine no action needed by you
- 1 : My appstate was updated, update your UI
- 2 : ...