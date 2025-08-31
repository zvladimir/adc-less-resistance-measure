# ADC-Free Resistance Sensor

Measure resistance-based signals using GPIO without an ADC.  
This project demonstrates how to read thermistors and other resistance-based sensors on STM32 (Rust firmware) by measuring the capacitor charge time through GPIO pins.

---

## How it works

The method uses a simple RC charging circuit and GPIO pins:

1. **Discharge phase**  
   - Configure `GP0` as output low (0).  
   - `GP1`, `GP2` are inputs.  
   - The capacitor discharges through `GP0`.

2. **Measurement phase (thermistor path)**  
   - Set `GP0` as input, `GP1` as output high (1).  
   - The capacitor charges through the thermistor and a parallel resistor `R_PAR`.  
   - Voltage on `GP0` rises until it crosses the digital threshold (~1.88 V).  
   - Measure the charging time `t1`.

3. **Reference measurement phase**  
   - Discharge the capacitor again.  
   - Set `GP0` as input, `GP2` as output high (1).  
   - The capacitor charges through the reference resistor `R_REF`.  
   - Measure the charging time `t2`.

4. **Calculation**  
   - The ratio `t1 / t2` corresponds to the ratio of the resistances `(R_THERMISTOR || R_PAR) / R_REF`.  
   - From this, the sensor resistance can be calculated.  

Since both measurements use the same capacitor, its absolute value does not affect accuracy.

---

## Example Results

| Nominal (Ω) | Measured (Ω) | Error (%) |
|-------------|--------------|-----------|
| 106         | 111          | 4.7       |
| 490         | 484          | 1.22      |
| 1116        | 1096         | 1.76      |
| 5077        | 5032         | 0.88      |
| 10009       | 10011        | 0.02      |
| 15055       | 15093        | 0.25      |
| 20015       | 20310        | 1.47      |

Estimated overall accuracy is about **±5%**, considering GPIO threshold variation and reference resistor tolerance.  
This is sufficient for many temperature-sensing applications.

---

## Applications

- Thermistor-based temperature measurement  
- Other resistance-based sensors (LDR, gas sensors, etc.)  
- Low-cost designs without ADC peripherals  

---

## Hardware Setup

- STM32F723IEK (tested, but can be ported to other MCUs)  
- One capacitor (value is not critical)  
- Thermistor (NTC/PTC) + optional parallel resistor `R_PAR`  
- Reference resistor `R_REF`  

---

## Firmware

The firmware is written in **Rust** for STM32.  
It measures charge times using GPIO state changes and simple timing functions.  

---

## License

MIT License  
