import { spawnSync } from "node:child_process"
import fs from "node:fs"
import path from "node:path"

const buildDir = "./dist"

spawnSync("rm", ["-rf", buildDir], { stdio: "inherit" })
spawnSync("tsc", ["-b", "./scripts/tsconfig.cjs.json", "./scripts/tsconfig.esm.json", "./scripts/tsconfig.types.json"], { stdio: "inherit" })

function createEsmModulePackageJson() {
  fs.readdir(buildDir, function (err, dirs) {
    if (err) {
      throw err
    }
    dirs.forEach(function (dir) {
      if (dir === "esm") {
        // 1. add package.json file with "type": "module"
        var packageJsonFile = path.join(buildDir, dir, "/package.json")
        if (!fs.existsSync(packageJsonFile)) {
          fs.writeFileSync(
            packageJsonFile,
            '{"type": "module"}',
            'utf8',
            err => { if (err) throw err }
          )
        }
      }
    })
  })
}

createEsmModulePackageJson()
