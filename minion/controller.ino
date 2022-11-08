struct Motor
{
  int pin_forward;
  int pin_backward;
  int pin_speed;
};

// Motor pins
const Motor front_left = {8,7,9};
const Motor front_right = {12,11,10};
const Motor back_left = {5,4,6};
const Motor back_right = {1,2,3};

void setupMotor(Motor motor) {
  pinMode(motor.pin_forward, OUTPUT);
  pinMode(motor.pin_backward, OUTPUT);
  pinMode(motor.pin_speed, OUTPUT);
}

void runMotor(Motor motor) {
  int direction = Serial.read();
  int speed = Serial.read();

  if (direction == 1)
    {
      digitalWrite(motor.pin_forward, HIGH);
      digitalWrite(motor.pin_backward, LOW);
    }
    else
    {
      digitalWrite(motor.pin_forward, LOW);
      digitalWrite(motor.pin_backward, HIGH);
    }
    analogWrite(motor.pin_speed, speed);
}


void setup()
{
  pinMode(LED_BUILTIN, OUTPUT);

  setupMotor(front_left);
  setupMotor(front_right);
  setupMotor(back_left);
  setupMotor(back_right);

  Serial.begin(115200);
  while (!Serial)
    ;
}

void loop()
{
  if (Serial.available() >= 8)
  {
    runMotor(front_left);
    runMotor(front_right);
    runMotor(back_left);
    runMotor(back_right); 

    Serial.write('Y');
  }
}
