// CONFIG

// The time from last message before motors stops. In miliseconds.
const int FAILSAFE = 500; 

// How often we should update the speed of the motors. In miliseconds.
const int MOTOR_UPDATE = 50;
// Maximum step toward the goal in one update cycle in one go from -255 to 255
const int MOTOR_STEP = 50;

// TYPES
struct Motor
{
  const int pin_forward;
  const int pin_backward;
  const int pin_speed;
  int current;
  int goal;
  int update;
};


// GLOBALS

unsigned long current_loop = 0;
unsigned long serial_update = 0;

Motor front_left = {8,7,9,0,0,0};
Motor front_right = {12,11,10,0,0,0};
Motor back_left = {5,4,6,0,0,0};
Motor back_right = {1,2,3,0,0,0};


// FUNCTIONS
void setupTime() {
  current_loop = millis();
}

int updateTime() {
  unsigned long last_loop = current_loop;
  current_loop = millis();
  return current_loop - last_loop;
}

void testFallback(int step_time) {
  serial_update += step_time;
  if (serial_update >= FAILSAFE) {
    front_left.goal = 0;
    front_right.goal = 0;
    back_left.goal = 0;
    back_right.goal = 0;
  }
}

void setupMotor(Motor &motor) {
  pinMode(motor.pin_forward, OUTPUT);
  pinMode(motor.pin_backward, OUTPUT);
  pinMode(motor.pin_speed, OUTPUT);
}

void setupMotors() {
  setupMotor(front_left);
  setupMotor(front_right);
  setupMotor(back_left);
  setupMotor(back_right);
}

void applyMotor(Motor &motor){
  if (motor.current >= 0)
  {
    digitalWrite(motor.pin_forward, HIGH);
    digitalWrite(motor.pin_backward, LOW);
  }
  else
  {
    digitalWrite(motor.pin_forward, LOW);
    digitalWrite(motor.pin_backward, HIGH);
  }
  analogWrite(motor.pin_speed, abs(motor.current));
}

void updateMotor(Motor &motor, int step_time){
  motor.update += step_time;
  if (motor.update >= MOTOR_UPDATE) {
    motor.update -= MOTOR_UPDATE;

    // Update current
    if (motor.current == motor.goal) {
      // there is nothing to update
      return;
    }

    int diff = motor.goal - motor.current;
    int step = constrain(diff, -MOTOR_STEP, MOTOR_STEP);
    motor.current += step;
    applyMotor(motor);
  }
}

void updateMotors(int step_time) {
  updateMotor(front_left, step_time);
  updateMotor(front_right, step_time);
  updateMotor(back_left, step_time);
  updateMotor(back_right, step_time);
}

void setupSerial() {
  Serial.begin(115200);
  while (!Serial)
    ;
}

void updateGoal(Motor &motor) {
  int forward = Serial.read();
  int speed = Serial.read();
  int goal = speed;
  if (forward != 1) {
    goal *= -1;
  }
  motor.goal = goal;
}

void updateSerial() {
  if (Serial.available() >= 8)
  {
    serial_update = 0;

    updateGoal(front_left);
    updateGoal(front_right);
    updateGoal(back_left);
    updateGoal(back_right); 

    Serial.write('Y');
  }
}

// MAIN

void setup()
{
  setupSerial();
  setupMotors();
  setupTime();

}

void loop()
{
  int step_time = updateTime();
  testFallback(step_time);
  updateMotors(step_time);
  updateSerial();
}
