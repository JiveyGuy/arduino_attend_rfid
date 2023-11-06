# 1 "/home/jivey/Devel/arduino_attend_rfid/arduino/arduino.ino"
# 2 "/home/jivey/Devel/arduino_attend_rfid/arduino/arduino.ino" 2




MFRC522 mfrc522(10, 9); // Create MFRC522 instance.


// function headers
void setup();
void loop();
// this function returns the id of the card
String readCard();

void setup() {
  Serial.begin(9600); // Initiate a serial communication
  SPI.begin(); // Initiate  SPI bus
  mfrc522.PCD_Init(); // Initiate MFRC522
}

// Main loop.
void loop() {
  // check if we got a serial command
  if (Serial.available() > 0) {
    String command = Serial.readStringUntil('\n');
    // switch on the command
    if (command == "read") {
      String card = readCard();
      if (card.length() > 0) {
        Serial.println(card);
      }
      else {
        Serial.println("no card");
      }
    }
    else if (command == "ping") {
      Serial.println("pong");
    }
  }
}

// read the id of the card
String readCard() {
  // Look for new cards
  if ( ! mfrc522.PICC_IsNewCardPresent()) {
    return "";
  }
  // Select one of the cards
  if ( ! mfrc522.PICC_ReadCardSerial()) {
    return "";
  }
  // get the id of the card
  String content = "";
  byte letter;
  for (byte i = 0; i < mfrc522.uid.size; i++) {
     content.concat(String(mfrc522.uid.uidByte[i] < 0x10 ? " 0" : " "));
     content.concat(String(mfrc522.uid.uidByte[i], 16));
  }
  content.toUpperCase();
  return content.substring(1);
}
