import { test, expect } from 'vitest';
import { startProjectAndTest } from '../../e2e/vitestSetup';
import { basename, dirname } from 'path';
import { fileURLToPath } from 'url';

const name = basename(import.meta.url);
const projectPath = dirname(fileURLToPath(import.meta.url));

test(`e2e tests - ${name}`, async () => {
  const runTest = (command?: 'start' | 'preview') =>
    startProjectAndTest(
      projectPath,
      async (page) => {
        await page.waitForSelector('.virtual-module', {
          timeout: 10000
        });
        const root = await page.$('.virtual-module');
        const innerHTML = await root?.innerHTML();
        expect(innerHTML).toContain('virtual-module');
      },
      command
    );

  await runTest();
});
