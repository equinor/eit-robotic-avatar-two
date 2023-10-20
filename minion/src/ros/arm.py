import rclpy
from open_manipulator_msgs.srv import SetJointPosition
from rclpy.node import Node


def map(value, outputMin, outputMax, inputMin = -0.5, inputMax = 0.5):
    # Figure out how 'wide' each range is
    inputSpan = inputMax - inputMin
    outputSpan = outputMax - outputMin

    # Convert the input range into a 0-1 range (float)
    valueScaled = float(value - inputMin) / float(inputSpan)

    # Convert the 0-1 range into a value in the output range.
    return round(outputMin + (valueScaled * outputSpan),3)

def clamp(num, min_value, max_value):
   return max(min(num, max_value), min_value)


class MyClient(Node):
    def __init__(self):
        super().__init__('my_client')
        
        # Create Service Clients
        self.cli_joint = self.create_client(SetJointPosition, 'goal_joint_space_path')
        self.cli_tool = self.create_client(SetJointPosition, 'goal_tool_control')
        while not self.cli_joint.wait_for_service(timeout_sec=1.0):
            self.get_logger().info('service not available, waiting again...')
        self.req = SetJointPosition.Request()
        self.req_tool = SetJointPosition.Request()

    def send_request(self, data):
        # Translate data to radians
        rx = map(clamp(data['rx'], -0.5, 0.5), outputMin=1.57, outputMax=-1.57) 
        ry = map(clamp(data['ry'], -0.5, 0.5), outputMin=-1.57, outputMax=1.57)
        rz = map(clamp(data['rz'], -0.25, 0.25), outputMin=-0.01, outputMax=0.01)

        # # Set joint states
        self.req.joint_position.joint_name = ['joint1', 'joint2', 'joint3', 'joint4']
        self.req.joint_position.position = [ry, -1.33, 0.367, rx + 0.93]
        self.req.path_time = 0.15

        # Set gripper/tool state
        self.req_tool.joint_position.joint_name = ['gripper']
        self.req_tool.joint_position.position = [rz - 0.004]
        self.req_tool.path_time = 0.15

        # Spin the nodes
        self.future_tool = self.cli_tool.call_async(self.req_tool)
        rclpy.spin_until_future_complete(self, self.future_tool)
        self.future = self.cli_joint.call_async(self.req)
        rclpy.spin_until_future_complete(self, self.future)

        return self.future.result()

def arm_start():
    rclpy.init()
    return MyClient()

def arm_run(arm, data):
    arm.send_request(data)
