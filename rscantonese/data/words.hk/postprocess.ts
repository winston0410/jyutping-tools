// NOTE FlatAction will run this script from root
// cwd: /home/runner/work/jyutping-tools/jyutping-tools
const cwd = Deno.cwd();

console.log(`cwd: ${cwd}`);

const ALL_DATA_ZIP_PATH = `${cwd}/rscantonese/data/words.hk/all.csv.gz`;

const unzipProcess = Deno.run({
  cmd: ["gunzip", ALL_DATA_ZIP_PATH],
  stdout: "piped",
  stderr: "piped",
});

try {
    const output = await unzipProcess.output()
    const error = await unzipProcess.stderrOutput()
    console.log(`check unzip stdout ${output}`)
    console.log(`check unzip stderr ${error}`)
} catch (error) {
    console.log('unexpected error: ', error)
}
