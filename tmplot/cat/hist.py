from dataclasses import dataclass

from .common import CommonPlotter


@dataclass
class Hist(CommonPlotter):
    def run(self) -> None:
        if self.args.labels is None:
            for data in self.data:
                self.ax1.hist(data[:, 0])
        else:
            for data, label in zip(self.data, self.labels):
                self.ax1.hist(data[:, 0], label=label)
        self.save()
