// const { exec } = require("child_process");
import { execSync } from 'child_process';
import * as fs from 'fs';
import * as path from 'path';
import * as dotenv from 'dotenv';
import { PrismaClient } from '@prisma/client';

async function main() {
    dotenv.config();
    const dirname = path.resolve();
    const dumpList = fs.readFileSync(dirname + '/prisma/dump-list.txt', 'utf8').split('\n');

    const outputDir = "/prisma/seed/";
    let connectionString =  process.env.DATABASE_URL;
    if(connectionString?.includes('?')) {
        connectionString = connectionString?.split('?')[0];
    }

    const db = new PrismaClient();
    const wojtek = await db.user.findUnique({
        where: {
            username: 'frondeus'
        }
    });

    let i = 0;
    for(const dumpItem of dumpList) {
        i += 1;
        console.log("Dump: ", dumpItem);
        const filename = dirname + outputDir + i.toString().padStart(4, '0') + '-' + dumpItem.toLowerCase() + '.sql';
        execSync(`pg_dump ${connectionString} -n public -t \\"${dumpItem}\\" -a --no-comments --inserts -f ${filename}`);

        execSync(`sed -i 's/^SET.*$//g' ${filename}`);
        execSync(`sed -i 's/^--.*$//g' ${filename}`);
        if (wojtek !== null) {
            execSync(`sed -i 's/${wojtek.id}/\\{\\{userId}\\}/g' ${filename}`);
        }
        execSync(`sed -i 's/^SELECT pg_catalog.set_config.*$//g' ${filename}`);
        execSync(`sed -i '/^$/N;/^\\n$/D' ${filename}`);
    }
}

main()
    .catch(async (e) => {
        console.error(e)
        process.exit(1)
    })