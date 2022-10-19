const int pin_front_left_forward = 2;
const int pin_front_left_backwards = 1;
const int pin_front_left_speed = 3;
const int pin_front_right_forward = 4;
const int pin_front_right_backwards = 5;
const int pin_front_right_speed = 6;
const int pin_back_left_forward = 7;
const int pin_back_left_backwards = 8;
const int pin_back_left_speed = 9;
const int pin_back_right_forward = 11;
const int pin_back_right_backwards = 12;
const int pin_back_right_speed = 10;

void setup()
{
  pinMode(LED_BUILTIN, OUTPUT);

  pinMode(pin_front_left_forward, OUTPUT);
  pinMode(pin_front_left_backwards, OUTPUT);
  pinMode(pin_front_left_speed, OUTPUT);

  Serial.begin(115200);
  while (!Serial)
    ;
}

void loop()
{
  if (Serial.available() >= 8)
  {
    int front_left_direction = Serial.read();
    int front_left_speed = Serial.read();
    int front_right_direction = Serial.read();
    int front_right_speed = Serial.read();
    int back_left_direction = Serial.read();
    int back_left_speed = Serial.read();
    int back_right_direction = Serial.read();
    int back_right_speed = Serial.read();

    if (front_left_direction == 1)
    {
      digitalWrite(pin_front_left_forward, HIGH);
      digitalWrite(pin_front_left_backwards, LOW);
    }
    else
    {
      digitalWrite(pin_front_left_forward, LOW);
      digitalWrite(pin_front_left_backwards, HIGH);
    }
    analogWrite(pin_front_left_speed, front_left_speed);

    if (front_right_direction == 1)
    {
      digitalWrite(pin_front_right_forward, HIGH);
      digitalWrite(pin_front_right_backwards, LOW);
    }
    else
    {
      digitalWrite(pin_front_right_forward, LOW);
      digitalWrite(pin_front_right_backwards, HIGH);
    }
    analogWrite(pin_front_right_speed, front_right_speed);

    if (back_left_direction == 1)
    {
      digitalWrite(pin_back_left_forward, HIGH);
      digitalWrite(pin_back_left_backwards, LOW);
    }
    else
    {
      digitalWrite(pin_back_left_forward, LOW);
      digitalWrite(pin_back_left_backwards, HIGH);
    }
    analogWrite(pin_back_left_speed, back_left_speed);

    if (back_right_direction == 1)
    {
      digitalWrite(pin_back_right_forward, HIGH);
      digitalWrite(pin_back_right_backwards, LOW);
    }
    else
    {
      digitalWrite(pin_back_right_forward, LOW);
      digitalWrite(pin_back_right_backwards, HIGH);
    }
    analogWrite(pin_back_right_speed, back_right_speed);
    
    Serial.write('Y');
  }
}
