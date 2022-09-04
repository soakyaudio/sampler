import numpy as np
import soundfile as sf

# Create 1s sine test sample (480hz left, 240hz right).
sample_rate = 48000
t = np.linspace(0, 1, sample_rate * 1)
y_left = np.sin(2 * np.pi * 480 * t)
y_right = np.sin(2 * np.pi * 240 * t)
y = np.column_stack((y_left, y_right))
sf.write("test_sine.wav", y, sample_rate, "PCM_16")
