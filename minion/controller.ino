struct Motor
{
  int pin_forward;
  int pin_backward;
  int pin_speed;
};

// Motor pins
const Motor front_left = {2,1,3};
const Motor front_right = {4,5,6};
const Motor back_left = {7,8,9};
const Motor back_right = {11,12,10};

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
      digitalWrite(motor.pin_forward, HIGH);
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
