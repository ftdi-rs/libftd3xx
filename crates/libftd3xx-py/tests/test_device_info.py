import unittest
import libftd3xx as ftd3xx

class DeviceInfoTestCase(unittest.TestCase):
    def setUp(self):
        pass

    def test_create_device_info_list(self):
        _ = ftd3xx.create_device_info_list()

    def test_get_device_info_list(self):
        device_count = ftd3xx.create_device_info_list()
        device_info = ftd3xx.get_device_info_list(device_count)
        self.assertEqual(device_count, len(device_info))