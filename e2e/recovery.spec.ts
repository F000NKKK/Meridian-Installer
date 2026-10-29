import { expect, test } from '@playwright/test';
import { host } from './host';

test('uninstalled product leaves Installed and returns to Available', async ({ page }) => {
  await host(page, { installed: true });
  await page.getByRole('button', { name: 'More', exact: true }).click();
  await page.getByText('Uninstall', { exact: true }).click();
  await page.getByRole('dialog').getByRole('button', { name: 'Uninstall', exact: true }).click();
  await expect(page.getByRole('heading', { name: 'Meridian IDLE', exact: true })).toBeHidden();
  await page.getByText('Available', { exact: true }).click();
  await expect(page.getByRole('heading', { name: 'Meridian IDLE', exact: true })).toBeVisible();
});

test('self update shows its failure and can be retried', async ({ page }) => {
  await host(page, { selfUpdate: true });
  const dialog = page.getByRole('dialog', { name: 'Meridian Installer needs to be updated' });
  await dialog.getByRole('button', { name: 'Update', exact: true }).click();
  await page.evaluate(() => (window as any).__installerTest.emit({ kind: 'finished', ok: false, cancelled: false, error: 'Checksum mismatch' }, 'meridian.installer'));
  await expect(dialog.getByText('Installation failed: Checksum mismatch')).toBeVisible();
  await dialog.getByRole('button', { name: 'Update', exact: true }).click();
  await expect.poll(() => page.evaluate(() => (window as any).__installerTest.calls.filter((c: any) => c.command === 'selfUpdate').length)).toBe(2);
});

test('failure to start the updated installer offers a launch retry', async ({ page }) => {
  await host(page, { selfUpdate: true });
  await page.evaluate(() => {
    const h = (window as any).__installerTest;
    const launch = h.api.launch;
    let first = true;
    h.api.launch = async (product: string) => {
      if (first) { first = false; throw new Error('Launch denied'); }
      return launch(product);
    };
  });
  const dialog = page.getByRole('dialog', { name: 'Meridian Installer needs to be updated' });
  await dialog.getByRole('button', { name: 'Update', exact: true }).click();
  await page.evaluate(() => (window as any).__installerTest.emit({ kind: 'finished', ok: true, cancelled: false, error: null }, 'meridian.installer'));
  await expect(dialog.getByText('Error: Launch denied')).toBeVisible();
  await dialog.getByRole('button', { name: 'Launch', exact: true }).click();
  await expect.poll(() => page.evaluate(() => (window as any).__installerTest.calls.some((c: any) => c.command === 'launch'))).toBe(true);
});

test('failed pause shows the error without pretending the download is paused', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  await host(page, { installed: true });
  await page.evaluate(() => { (window as any).__installerTest.api.pause = async () => { throw new Error('Pause denied'); }; });
  await page.getByRole('button', { name: 'Update', exact: true }).click();
  await page.getByRole('button', { name: 'Pause', exact: true }).click();
  await expect(page.getByText('Error: Pause denied')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Pause', exact: true })).toBeVisible();
  expect(errors).toEqual([]);
});

test('failed cancellation leaves the operation running and reports its error', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  await host(page, { installed: true });
  await page.evaluate(() => { (window as any).__installerTest.api.cancel = async () => { throw new Error('Cancel denied'); }; });
  await page.getByRole('button', { name: 'Update', exact: true }).click();
  await page.getByRole('button', { name: 'Cancel', exact: true }).click();
  await expect(page.getByText('Error: Cancel denied')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Pause', exact: true })).toBeVisible();
  expect(errors).toEqual([]);
});

test('release refresh reports a failure and can be retried', async ({ page }) => {
  await host(page, { installed: true });
  await page.evaluate(() => { (window as any).__installerTest.api.snapshot = async () => { throw new Error('Catalog offline'); }; });
  const refresh = page.getByRole('button', { name: 'Check for updates', exact: true });
  await refresh.click();
  await expect(page.getByText('Error: Catalog offline')).toBeVisible();
  await expect(refresh).toBeEnabled();
  await page.evaluate(() => {
    const h = (window as any).__installerTest;
    h.api.snapshot = async () => structuredClone(h.snapshot);
  });
  await refresh.click();
  await expect(page.getByText('Updates available: 1.')).toBeVisible();
});
