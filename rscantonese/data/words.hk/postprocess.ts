// NOTE FlatAction will run this script from root
// cwd: /home/runner/work/jyutping-tools/jyutping-tools
const cwd = Deno.cwd();

console.log(`cwd: ${cwd}`);

const ALL_DATA_ZIP_PATH = `${cwd}/rscantonese/data/words.hk/all.csv.gz`;

const unzipProcess = Deno.run({
  cmd: ["gunzip", ALL_DATA_ZIP_PATH, "-f"],
  stdout: "piped",
  stderr: "piped",
});

try {
  const { code } = await unzipProcess.status();

  if (code !== 0) {
    const stderr = await unzipProcess.stderrOutput();
    const error = new TextDecoder().decode(stderr);
    console.log(`unzip stderr: ${error}`);
  }
} catch (error) {
  console.log("unexpected error: ", error);
}
