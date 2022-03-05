## Optimization reference

https://github.com/johnthagen/min-sized-rust

## Workspace

Right now the workspace is not usable, as code in `web/package.json` has been changed to accomodate the deployment on Vercel.

### For NPM workspace

```json
  "dependencies": {
    "jyutping-helpers": "file:jyutping-helpers"
  }
```

### For Vercel deployment

```json
  "dependencies": {
    "jyutping-helpers": "file:../jyutping-helpers"
  }
```
