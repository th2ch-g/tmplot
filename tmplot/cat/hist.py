from dataclasses import dataclass

from .common import CommonPlotter


@dataclass
class Hist(CommonPlotter):
    def run(self) -> None:
        if self.data[0].ndim == 1:
            data = self.data
        else:
            data = []
            for d in self.data:
                data.append(d[:, self.use_hist_idx])
        if self.args.labels is None:
            for d in data:
                self.ax1.hist(d)
        else:
            for d, label in zip(data, self.labels):
                self.ax1.hist(d, label=label)
        self.save()
