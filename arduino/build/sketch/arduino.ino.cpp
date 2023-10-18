#include <Arduino.h>
#line 1 "/home/jivey/Devel/arduino_attend_rfid/arduino/arduino.ino"
// this is an arduino program that prints
// text recieved over serial from the PC

#line 4 "/home/jivey/Devel/arduino_attend_rfid/arduino/arduino.ino"
void setup();
#line 9 "/home/jivey/Devel/arduino_attend_rfid/arduino/arduino.ino"
void loop();
#line 4 "/home/jivey/Devel/arduino_attend_rfid/arduino/arduino.ino"
void setup()
{
  Serial.begin(9600);
}

void loop()
{
  if (Serial.available())
  {
    Serial.write(Serial.read());
  }
}
