#!/usr/bin/env python3

DEFAULT_REM = 18
RETINA_REM = 15
RETINA_RATIO = 15 / 18
# round to nearest half-integer
ROUNDING_PRECISION = 2


def retina_proportion(measurement: float) -> float:
    return round(measurement * RETINA_RATIO * ROUNDING_PRECISION) / ROUNDING_PRECISION


def convert() -> None:
    measurement = float(input("Original: "))
    retina = retina_proportion(measurement)
    print(f"Retina:   \033[1m\033[92m{retina}\033[0m")


if __name__ == "__main__":
    try:
        while True:
            convert()
    except KeyboardInterrupt:
        pass
