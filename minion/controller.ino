void setup() {
  pinMode(LED_BUILTIN, OUTPUT);
  Serial.begin(115200);
  while (!Serial); 
}

void loop() {
  if (Serial.available() >= 8) {
    int front_left_direction = Serial.read();
    int front_left_speed = Serial.read();
    int front_right_direction = Serial.read();
    int front_right_speed = Serial.read();
    int back_left_direction = Serial.read();
    int back_left_speed = Serial.read();
    int back_right_direction = Serial.read();
    int back_right_speed = Serial.read();

    if (front_left_direction == 1) {
      digitalWrite(LED_BUILTIN, HIGH);
    } else {
      digitalWrite(LED_BUILTIN, LOW);
    }
  }
}
