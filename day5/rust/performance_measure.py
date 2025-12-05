import subprocess

times = []
for i in range(100):
    output = subprocess.run(
        [
            "cargo run --release",
        ],
        shell=True,
        check=True,
        capture_output=True,
    )

    times.append(float(output.stdout.decode().splitlines()
                 [-1].split(' ')[-1][:-2]))

avg_time = sum(times) / len(times)
print(f"Average time over 100 runs: {avg_time:.2f} ms")
