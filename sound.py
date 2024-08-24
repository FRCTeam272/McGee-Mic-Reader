import sounddevice as sd
import numpy as np


def read_decibel_levels(sample_rate=44100, duration=1) -> np.ndarray:
    """
    Reads the decibel levels from the recorded audio.

    Parameters:
    - sample_rate (int): The sample rate of the audio recording. Default is 44100.
    - duration (int): The duration of the audio recording in seconds. Default is 1.

    Returns:
    - decibel_levels (numpy.ndarray): An array of decibel levels calculated from the recorded audio.
    """

    # Record audio from the default microphone
    audio = sd.rec(int(sample_rate * duration), samplerate=sample_rate, channels=1)

    # Wait for the recording to complete
    sd.wait()

    # Calculate the decibel levels from the recorded audio
    decibel_levels = 20 * np.log10(np.abs(audio))

    return decibel_levels

def compare(base_line, test_line):
    """
    Compares the decibel levels of the base line and the test line.

    Parameters:
    - base_line (numpy.ndarray): An array of decibel levels from the base line.
    - test_line (numpy.ndarray): An array of decibel levels from the test line.

    Returns:
    - result (float): The difference in decibel levels between the base line and the test line.
    """

    # Calculate the difference in decibel levels between the base line and the test line
    result = np.mean(base_line) - np.mean(test_line)

    return abs(result)


