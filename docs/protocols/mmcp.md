|     |     |     |
| --- | --- | --- |
|  | Mud Master Chat Protocol |  |

|     |
| --- |

| [![MUDhalla is a website dedicated to multiplayer text adventure games, also known as MUDs.](https://mudhalla.net/mudhalla_logo.png)](https://mudhalla.net/) |

|     |
| --- |
|  |

|     |
| --- |
|  |
|  |
| The MMCP Protocol |
|  |
|  |
|  |
| MMCP is a decentralized chat protocol which allows MUD clients to communicate with each other over a TCP/IP connection. |
|  |
|  |
|  |
| **Establishing a Connection** |
|  |
| * * * |
|  |
| **Caller:**<br>When a TCP/IP connection is made the caller sends a connection handshake which looks as following: "CHAT:<chat name>\\n<ip address><port>". The sprintf syntax is: "CHAT:%s\\n%s%-5u". The port must be 5 characters, padded on the right side with spaces. Once this string has been sent it waits for a response from the receiver. If a "NO" is received the call is cancelled. If the call was accepted the string "YES:<chat name>\\n" is received.<br>**Receiver:**<br>When a socket call is detected it accepts the socket then waits for the "CHAT:" string to be sent from the caller. If the receiver wishes to deny the call, the string "NO" needs to be sent back to the caller. To accept the call, the string "YES:<chat name>\\n" is sent back. |
|  |
| The quote characters mean that the encased words are a string, the quotes themselves should not be send. |
|  |
|  |
|  |
| **Chat Data Blocks** |
|  |
| * * * |
|  |
| A chat data block looks as following: <COMMAND BYTE><data><END OF COMMAND>. <br>All chat data needs to follow this format except for the initial connection handshake and file transfer blocks which are a fixed size and don't need the <END OF COMMAND> byte. |
|  |
|  |
|  |
| **List of the <COMMAND BYTE> values** |
|  |
| * * * |
|  |
| ```<br>CHAT_NAME_CHANGE                      1<br>CHAT_REQUEST_CONNECTIONS              2<br>CHAT_CONNECTION_LIST                  3<br>CHAT_TEXT_EVERYBODY                   4<br>CHAT_TEXT_PERSONAL                    5<br>CHAT_TEXT_GROUP                       6<br>CHAT_MESSAGE                          7<br>CHAT_DO_NOT_DISTURB                   8<br>CHAT_VERSION                         19<br>CHAT_FILE_START                      20<br>CHAT_FILE_DENY                       21<br>CHAT_FILE_BLOCK_REQUEST              22<br>CHAT_FILE_BLOCK                      23<br>CHAT_FILE_END                        24<br>CHAT_FILE_CANCEL                     25<br>CHAT_PING_REQUEST                    26<br>CHAT_PING_RESPONSE                   27<br>CHAT_PEEK_CONNECTIONS                28<br>CHAT_PEEK_LIST                       29<br>CHAT_SNOOP_START                     30<br>CHAT_SNOOP_DATA                      31<br>CHAT_END_OF_COMMAND                 255<br>``` |
|  |
|  |
|  |
| **MMCP Commands** |
|  |
| * * * |
|  |
| <CHAT\_NAME\_CHANGE><new name><CHAT\_END\_OF\_COMMAND> |
|  |
| When a user changes their chat name the new name needs to be broadcast to all of their connections. |
|  |
| <CHAT\_REQUEST\_CONNECTIONS><CHAT\_END\_OF\_COMMAND> |
|  |
| Requests connections from another connection asking to see all the people that person has marked as public, then try to connect to all of those yourself. |
|  |
| <CHAT\_CONNECTION\_LIST><address>,<port>,<address>,<port><CHAT\_END\_OF\_COMMAND> |
|  |
| The receiver needs to put all the IPs and port numbers of the public connections in a comma delimited string and send them back as a connection list. |
|  |
| <CHAT\_TEXT\_EVERYBODY><chat message><CHAT\_END\_OF\_COMMAND> |
|  |
| Used to send a chat message to everybody. The chat message needs to be generated on the sender's side, including the line feeds and the "<chat name> chats to everybody" string.<br>**Receiver:**<br>If the chat connection isn't being ignored, you simply print the string. If you have any connections marked as being served you need to echo this string to those connections. Or if this is coming from a connection being served, you need to echo to all your other connections. This allows people who cannot connect directly to each other to connect with a 3rd person who \*can\* connect to both and be a server for them. |
|  |
| <CHAT\_TEXT\_PERSONAL><chat message><CHAT\_END\_OF\_COMMAND> |
|  |
| This works the same way as CHAT\_TEXT\_EVERYBODY. The text should be changed so the receiver knows this was a personal chat and not broadcast to everybody.<br>**Receiver:**<br>Just print the incoming message if you aren't ignoring this connection. |
|  |
| <CHAT\_TEXT\_GROUP><group><chat message><CHAT\_END\_OF\_COMMAND> |
|  |
| The group name is a 15 character string. It must be 15 characters long and padded on the right with spaces to fill it out. The text should be changed so the receiver knows this was a group chat and not broadcast to everybody.<br>**Receiver:**<br>Just print the incoming message if you aren't ignoring this connection. |
|  |
| <CHAT\_MESSAGE><message><CHAT\_END\_OF\_COMMAND> |
|  |
| This should be used for generic system messages, like accepting or declining a chat connection, and marking someone public or private. To let the other side know the message is generated from the chat program it is a good idea to make the string resemble something like: "\\n<CHAT> %s has refused your connection because your name is too long.\\n"<br>**Receiver:**<br>Just print the message string. |
|  |
| <CHAT\_VERSION><version string><CHAT\_END\_OF\_COMMAND> |
|  |
| The version string should include your mud client's name and version. |
|  |
| <CHAT\_FILE\_START><filename,length><CHAT\_END\_OF\_COMMAND> |
|  |
| Used to send a chat connection a file. The filename should not include the path. Length is the size of the file in bytes.<br>**Receiver:**<br>First should check if you are allowing files from this connection. Make sure the filename is valid and that the length was transmitted. If for any reason the data isn't valid or you don't want to accept files from this person a CHAT\_FILE\_DENY should be sent back to abort the transfer. If you want to continue you need to start the transfer by requesting a block of data with CHAT\_FILE\_BLOCK\_REQUEST. |
|  |
| <CHAT\_FILE\_DENY><message><CHAT\_END\_OF\_COMMAND> |
|  |
| Used when a CHAT\_FILE\_START has been received and you want to prevent the transfer from continuing. <message> is a string telling the reason it was denied. For example, if the file already exists you might deny it with: "File already exists."<br>**Receiver:**<br>Print the deny message. Deal with cleaning up any files you opened when you began the transfer. |
|  |
| <CHAT\_FILE\_BLOCK\_REQUEST><CHAT\_END\_OF\_COMMAND> |
|  |
| Sent to request the next block of data in a transfer.<br>**Receiver:**<br>Create a file block to be sent back. File blocks are fixed length so you must not add the CHAT\_END\_OF\_COMMAND byte. If the end of file is reached you need to send a CHAT\_FILE\_END, close the file, and let the user know the file transfer is completed. |
|  |
| <CHAT\_FILE\_BLOCK><block of data> |
|  |
| A file block is 500 bytes. No CHAT\_END\_OF\_COMMAND should be added.<br>**Receiver:**<br>The receiver needs to keep track of the number of bytes written to properly write the last block of data. The last block is unlikely to be the full 500 bytes. File transfers are receiver driven, so for each block of data you accept, you need to send another CHAT\_FILE\_BLOCK\_REQUEST back out to get more data. |
|  |
| <CHAT\_FILE\_END><CHAT\_END\_OF\_COMMAND> |
|  |
| Close up your files and be done with it. |
|  |
| <CHAT\_FILE\_CANCEL><CHAT\_END\_OF\_COMMAND> |
|  |
| Either side can send this command to abort a file transfer in progress. |
|  |
| <CHAT\_PING\_REQUEST><timing data><CHAT\_END\_OF\_COMMAND> |
|  |
| The timing data is up to the ping requester. |
|  |
| <CHAT\_PING\_RESPONSE><timing data><CHAT\_END\_OF\_COMMAND> |
|  |
| When receiving a CHAT\_PING\_REQUEST you should send back the timing data. |
|  |
| <CHAT\_PEEK\_CONNECTIONS><CHAT\_END\_OF\_COMMAND> |
|  |
| The sender requests connections from another connection asking to see all the people that person has marked as public. The receiver needs to put all the ip addresses, port numbers, and names in a tilde delimited string and send them back as a peek list. |
|  |
| <CHAT\_PEEK\_LIST><address>~<port>~<name>~<CHAT\_END\_OF\_COMMAND> |
|  |
| The receiver needs to put all the ip addresses, port numbers, and names in a tilde delimited string and send them back as a peek list. |
|  |
| <CHAT\_SNOOP\_START><CHAT\_END\_OF\_COMMAND> |
|  |
| The sender requests to start or stop snooping data from a chat connection. The Receiver decides whether to allow snooping or not. |
|  |
| <CHAT\_SNOOP\_DATA><message><CHAT\_END\_OF\_COMMAND> |
|  |
| Send by a mud client in snoop mode. The message should be echoed by the receiver, but not be further forwarded to avoid infinite loops. |
|  |
|  |
|  |
| **Links** |
|  |
|  |
|  |
| If you want a link added, you can email me at mudclient@gmail.com. |
|  |
|  |
|  |
| **Clients supporting MMCP** |
|  |
| * * * |
|  |
| [Comparison of MUD clients](http://en.wikipedia.org/wiki/Comparison_of_MUD_clients) \- Wikipedia contains an up to date list of the MMCP status of MUD clients. |
|  |
|  |
|  |
| **Snippets** |
|  |
| * * * |
|  |
| [TinTin++](https://tintin.mudhalla.net/) \- Open source MUD client supporting MMCP written in C. |
|  |
| [KildClient](https://www.kildclient.org/) \- Open source MUD client supporting an MMCP plugin written in Perl. |
|  |
| [Lua Chat](https://github.com/nickgammon/plugins/blob/master/Lua_Chat.xml) \- MUSHclient MMCP plugin written in Lua. |
|  |
|  |
|  |
