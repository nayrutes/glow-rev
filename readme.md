Reverse engineering tests for the USB-Protocol of the Ultimate Glow LED juggling clubs by Aerotech.

Findings:
Data transmission is done over write_bulk (endpoint: 0x01) and read_bulk (endpoint: 0x81)*. It is always first a write_transmission and then a read transmission.
The most common structure is a 6 byte message as kind of a header. The byte at index 0 is describing what we want to do. Byte idx 1 describes how long the (response) package is. Bytes idx 2 and 3 describes the "memory address" as little endian. The function of bytes idx 4 and 5 are unknown - so far only 0x00.

Start-signal:
*One exception is the start signal with write_control (request_type: 0x42 , request: 0xd1, value: 0x00, index: 0x00)

Set_color & stop_signal:
Setting the color while conneceted is done by transmitting [0x63, red_hex, green_hex, blue_hex]. This stops the playback and by using black as color, the leds are turned off.
The return for the read is [0x63, red_hex, green_hex, blue_hex].


Read_name:
The name is 64 bytes. So to retrieve it, we send 8 messages which each return 8 bytes of the name. [0x04, 0x08, first_byte at 0x80, 0x00, 0x00, 0x00]. the position should be changed by 0x80 + (i*message_id) where message_id counts up from 0 to 7.
The read contains the 6 bytes of the request and 8 bytes with the the partial name.

Set_name:
Writing the name works similar but the transmitted bytes are [0x05, 0x08, first_byte at 0x80, 0x00, 0x00, 0x00, ...8 bytes of partial name...]
The read contains one byte: 0x05


Read_file_name:
The same as read_name except with a different starting byte. [0x04, 0x08, first_byte at 0xc0, 0x00, 0x00, 0x00]

Set_file_name:
[0x05, 0x08, first_byte at 0xc0, 0x00, 0x00, 0x00, ...8 bytes of partial file_name...]

Read_group_name:
Group names are only 4 bytes long so one message is enough.
[0x04, 0x04, 0x7c, 0x00, 0x00, 0x00]

Set_group_name:
[0x05, 0x04, 0x7c, 0x00, 0x00, 0x00, ...4 bytes group-name...]


Read_program:
[0x01, 0x10, 2 bytes address L.E. starting at 0x00 0x40, 0x00, 0x00].

Write_program:
Writing the program has an additional step: Before of every 4 messages, a transmission is made with [0x03, 0x01, 2 bytes address L.E. starting at 0x00 0x40, 0x00, 0x00]. Why this is needed is not resoved but transmitting the program without it does not seem to work with inconclusive results.
The program itself is transmitted with [0x02, 0x10, 2 bytes address L.E. starting at 0x00 0x40, 0x00, 0x00, ...16 bytes of partial program...]


Program code:
Color:			[0x01, r, g, b]
Dealy (short):	[0x02, time]
Dealy (long):	[0x04, time 2 bytes L.E.]
Ramp (short): 	[0x0c, r, g, b, time]
Ramp (long): 	[0x0d, r, g, b, time 2 bytes L.E.]
Loop (start):	[0x03, count]
Loop (end):		[0x05]
END:			[0xff]
