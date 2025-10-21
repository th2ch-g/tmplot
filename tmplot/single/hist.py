from dataclasses import dataclass

from .common import CommonPlotter


@dataclass
class Hist(CommonPlotter):
    def run(self) -> None:
        if self.data.ndim == 1:
            data = self.data
        else:
            data = self.data[:, self.use_hist_idx]
        if self.label is None:
            self.ax.hist(
                data,
                bins=self.args.binsize,
            )
        else:
            self.ax.hist(
                data,
                bins=self.args.binsize,
                label=self.label,
            )
        self.save()
