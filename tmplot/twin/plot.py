from dataclasses import dataclass

from .common import CommonPlotter


@dataclass
class Plot(CommonPlotter):
    def run(self) -> None:
        if self.labels is None:
            self.ax2.plot(self.data[:, 0], self.data[:, 1])
            self.ax2.plot(self.data[:, 0], self.data[:, 2])
        else:
            self.ax2.plot(self.data[:, 0], self.data[:, 1], label=self.labels[0])
            self.ax2.plot(self.data[:, 0], self.data[:, 2], label=self.labels[1])
        self.save()
