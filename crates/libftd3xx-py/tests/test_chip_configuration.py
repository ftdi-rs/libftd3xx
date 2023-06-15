import unittest
import libftd3xx as ftd3xx

# import os
# input(f"OS PID: {os.getpid()}")


class ChipConfigurationTestCase(unittest.TestCase):
    def setUp(self):
        self.attribute_names = (
            "VendorID",
            "ProductID",
            "StringDescriptors",
            "bInterval",
            "PowerAttributes",
            "PowerConsumption",
            "Reserved2",
            "FIFOClock",
            "FIFOMode",
            "ChannelConfig",
            "OptionalFeatureSupport",
            "BatteryChargingGPIOConfig",
            "FlashEEPROMDetection",
            "MSIO_Control",
            "GPIO_Control",
        )

    def test_Ft60xConfiguration(self):
        config = ftd3xx.Ft60xConfiguration()
        
        for x, attribute_name in enumerate(self.attribute_names):
            attribute = getattr(config, attribute_name)
            # print(attribute_name, attribute)
            if isinstance(attribute, list):
                self.assertEqual(attribute, [0] * len(attribute), f"{attribute}")
                attribute = [x] * len(attribute)
                self.assertEqual(attribute, [x] * len(attribute), f"{attribute}")
            else:
                self.assertEqual(attribute, 0, f"{attribute}")
                attribute = x
                self.assertEqual(attribute, x, f"{attribute}")
            # print(attribute_name, attribute)

    def test_get_chip_configuration(self):
        device_count = ftd3xx.create_device_info_list()
        if device_count > 0:
            # device_info = ftd3xx.get_device_info_list(device_count)
            # handle = ftd3xx.create_by_serial_number(device_info[0].SerialNumber)
            handle = ftd3xx.create_by_index(0)
            _ = ftd3xx.get_chip_configuration(handle)
            #print(f"Chip Configuation: {_}")
            ftd3xx.close(handle)

    def test_set_chip_configuration(self):
        device_count = ftd3xx.create_device_info_list()
        if device_count > 0:
            # device_info = ftd3xx.get_device_info_list(device_count)
            # handle = ftd3xx.create_by_serial_number(device_info[0].SerialNumber)
            handle = ftd3xx.create_by_index(0)
            original_config = ftd3xx.get_chip_configuration(handle)
            #print(f"Chip Configuation: {original_config}")
            ftd3xx.set_chip_configuration(handle, original_config)
            new_config = ftd3xx.get_chip_configuration(handle)
            for x, attribute_name in enumerate(self.attribute_names):
                original_attribute = getattr(original_config, attribute_name)
                new_attribute = getattr(new_config, attribute_name)
                self.assertEqual(original_attribute, new_attribute, f"{attribute_name}")
                #print(attribute_name, original_attribute)
            ftd3xx.close(handle)


if __name__ == "__main__":
    unittest.main()
