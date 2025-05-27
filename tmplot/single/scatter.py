from dataclasses import dataclass

from .common import CommonPlotter


@dataclass
class Scatter(CommonPlotter):
    def run(self) -> None:
        if self.label is None:
            self.ax.scatter(self.data[:, 0], self.data[:, 1])
        else:
            self.ax.scatter(self.data[:, 0], self.data[:, 1], label=self.label)
        self.save()
