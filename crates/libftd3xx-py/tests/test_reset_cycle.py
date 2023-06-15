import unittest
import libftd3xx as ftd3xx

# import os
# input(f"OS PID: {os.getpid()}")


class ChipConfigurationTestCase(unittest.TestCase):
    def setUp(self):
        pass

    def test_reset_device_port(self):
        device_count = ftd3xx.create_device_info_list()
        if device_count > 0:
            # device_info = ftd3xx.get_device_info_list(device_count)
            # handle = ftd3xx.create_by_serial_number(device_info[0].SerialNumber)
            handle = ftd3xx.create_by_index(0)
            ftd3xx.reset_device_port(handle)
            ftd3xx.close(handle)

    def test_cycle_device_port(self):
        device_count = ftd3xx.create_device_info_list()
        if device_count > 0:
            # device_info = ftd3xx.get_device_info_list(device_count)
            # handle = ftd3xx.create_by_serial_number(device_info[0].SerialNumber)
            handle = ftd3xx.create_by_index(0)
            ftd3xx.cycle_device_port(handle)
            ftd3xx.close(handle)


if __name__ == "__main__":
    unittest.main()
