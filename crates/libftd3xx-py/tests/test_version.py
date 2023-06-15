import unittest
import libftd3xx as ftd3xx

# import os
# input(f"OS PID: {os.getpid()}")


class LibraryVersionTestCase(unittest.TestCase):
    def setUp(self):
        pass

    def test_get_library_version(self):
        _ = ftd3xx.get_library_version()


class DriverVersionTestCase(unittest.TestCase):
    def setUp(self):
        pass

    def test_get_driver_version(self):
        self.assertRaises(TypeError, ftd3xx.get_driver_version, (None,))

        device_count = ftd3xx.create_device_info_list()
        if device_count > 0:
            # device_info = ftd3xx.get_device_info_list(device_count)
            # handle = ftd3xx.create_by_serial_number(device_info[0].SerialNumber)
            handle = ftd3xx.create_by_index(0)
            _ = ftd3xx.get_driver_version(handle)
            #print(f"Driver Version: {_}")
            ftd3xx.close(handle)


if __name__ == '__main__':
    unittest.main()