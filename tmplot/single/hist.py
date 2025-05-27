from dataclasses import dataclass

from .common import CommonPlotter


@dataclass
class Hist(CommonPlotter):
    def run(self) -> None:
        if self.label is None:
            self.ax.hist(
                self.data,
                bins=self.args.binsize,
            )
        else:
            self.ax.hist(
                self.data,
                bins=self.args.binsize,
                label=self.label,
            )
        self.save()
