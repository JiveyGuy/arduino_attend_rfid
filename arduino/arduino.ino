// this is an arduino program that prints
// text recieved over serial from the PC

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