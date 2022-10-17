void setup() {
  pinMode(LED_BUILTIN, OUTPUT);
  Serial.begin(115200);
  while (!Serial); 
  Serial.println("Hello Computer");
}

void loop() {
  if (Serial.available() > 0) {
    int byte = Serial.read();
    if (byte > 0) {
      digitalWrite(LED_BUILTIN, HIGH);
    } else {
      digitalWrite(LED_BUILTIN, LOW);
    }
  }
}
