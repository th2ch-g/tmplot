from dataclasses import dataclass

from .common import CommonPlotter


@dataclass
class Plot(CommonPlotter):
    def run(self) -> None:
        if self.args.labels is None:
            for data in self.data:
                self.ax1.plot(data[:, 0], data[:, 1])
        else:
            for data, label in zip(self.data, self.labels):
                self.ax1.plot(data[:, 0], data[:, 1], label=label)
        self.save()
