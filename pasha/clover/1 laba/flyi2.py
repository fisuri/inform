# Information: https://clover.coex.tech/en/snippets.html#navigate_wait

from math import * 
import rospy
from clover import srv
from std_srvs.srv import Trigger
import numpy as np

rospy.init_node('flight')

get_telemetry = rospy.ServiceProxy('get_telemetry', srv.GetTelemetry)
navigate = rospy.ServiceProxy('navigate', srv.Navigate)
navigate_global = rospy.ServiceProxy('navigate_global', srv.NavigateGlobal)
set_position = rospy.ServiceProxy('set_position', srv.SetPosition)
set_velocity = rospy.ServiceProxy('set_velocity', srv.SetVelocity)
set_attitude = rospy.ServiceProxy('set_attitude', srv.SetAttitude)
set_rates = rospy.ServiceProxy('set_rates', srv.SetRates)
land = rospy.ServiceProxy('land', Trigger)

def navigate_wait(x=0, y=0, z=0, yaw=float('nan'), yaw_rate=0, speed=0.25, \
        frame_id='body', tolerance=0.3, auto_arm=False):

    res = navigate(x=x, y=y, z=z, yaw=yaw, yaw_rate=yaw_rate, speed=speed, \
        frame_id=frame_id, auto_arm=auto_arm)

    if not res.success:
        return res

    while not rospy.is_shutdown():
        telem = get_telemetry(frame_id='navigate_target')
        if sqrt(telem.x ** 2 + telem.y ** 2 + telem.z ** 2) < tolerance:
            return res
        rospy.sleep(0.2)

X = np.zeros(300)
Y = np.zeros(300)
j = 2*pi/300
for i in range(300):
    X[i] = cos(i*j) + cos(2*i*j) / 2
    Y[i] = sin(i*j) - sin(2*i*j) / 2

navigate_wait(x=0, y=0, z=1, frame_id='body', auto_arm=True)
for i in range(300):
    navigate_wait(x=X[i]+2, y=Y[i]+2, z=1, frame_id='aruco_map')

print('Land')
land()
