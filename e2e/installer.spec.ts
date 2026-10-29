import { expect, test } from '@playwright/test';
import { host } from './host';

test('notifications stay below the title bar, wrap, dismiss and close', async ({ page }) => {
  await host(page);
  const bell = page.getByTitle('Notifications', { exact: true });
  await bell.click();
  const panel = page.getByRole('dialog', { name: 'Notifications', exact: true });
  const bounds = await panel.boundingBox();
  const title = await page.locator('.mi-titlebar').boundingBox();
  expect(bounds!.y).toBeGreaterThanOrEqual(title!.y + title!.height);
  expect(bounds!.x).toBeGreaterThanOrEqual(0);
  expect(bounds!.x + bounds!.width).toBeLessThanOrEqual(1100);
  await panel.getByTitle('Dismiss').first().click();
  await expect(panel).not.toContainText('Catalog unavailable');
  await page.keyboard.press('Escape');
  await expect(panel).toBeHidden();
  await expect(bell).toBeFocused();
  await page.setViewportSize({ width: 900, height: 600 });
  await bell.click();
  await page.screenshot({ path: 'test-results/notifications.png' });
  await page.getByText('Installed', { exact: true }).click();
  await expect(panel).toBeHidden();
});

test('installation covers workloads, components, locations, pause and cancellation', async ({ page }) => {
  await host(page);
  await page.getByText('Available', { exact: true }).click();
  await page.getByRole('button', { name: 'Install', exact: true }).click();
  const dialog = page.getByRole('dialog', { name: 'Meridian IDLE' });
  await expect(dialog.getByRole('checkbox').first()).toBeChecked();
  await dialog.getByText('Individual components', { exact: true }).click();
  await expect(dialog.locator('.mi-component input')).toBeDisabled();
  await dialog.getByText('Installation locations', { exact: true }).click();
  await expect(dialog).toContainText('/home/test/.meridian');
  await dialog.getByRole('button', { name: 'Install', exact: true }).click();
  await page.getByRole('button', { name: 'Pause', exact: true }).click();
  await page.getByRole('button', { name: 'Resume', exact: true }).click();
  await page.getByRole('button', { name: 'Cancel', exact: true }).click();
  await expect(page.getByText('Cancelled.', { exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'Dismiss', exact: true }).click();
  await expect(page.getByText('Cancelled.', { exact: true })).toBeHidden();
});

test('pending, stale, failed and unread plans cannot authorize installation', async ({ page }) => {
  await host(page, { controlledPlans: true });
  await page.getByText('Available', { exact: true }).click();
  await page.getByRole('button', { name: 'Install', exact: true }).click();
  const dialog = page.getByRole('dialog', { name: 'Meridian IDLE' });
  const install = dialog.getByRole('button', { name: 'Install', exact: true });
  await expect.poll(() => page.evaluate(() => (window as any).__installerTest.plans.length)).toBe(1);
  await expect(install).toBeDisabled();
  await dialog.getByRole('checkbox').first().uncheck();
  await expect.poll(() => page.evaluate(() => (window as any).__installerTest.plans.length)).toBe(2);
  await page.evaluate(() => { const h = (window as any).__installerTest; h.plans[0].resolve(h.plan); });
  await expect(install).toBeDisabled();
  await page.evaluate(() => { const h = (window as any).__installerTest; h.plans[1].reject('Plan failed'); });
  await expect(dialog.getByText('Plan failed')).toBeVisible();
  await expect(install).toBeDisabled();
  await dialog.getByRole('checkbox').first().check();
  await expect.poll(() => page.evaluate(() => (window as any).__installerTest.plans.length)).toBe(3);
  await page.evaluate(() => { const h = (window as any).__installerTest; h.plans[2].resolve({ ...h.plan, unread: ['sdk'] }); });
  await expect(install).toBeDisabled();
});

test('installed product launches, updates and shows installation failure', async ({ page }) => {
  await host(page, { installed: true });
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  await expect.poll(() => page.evaluate(() => (window as any).__installerTest.calls.some((c: any) => c.command === 'launch'))).toBe(true);
  await page.getByRole('button', { name: 'Update', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Pause', exact: true })).toBeVisible();
  await page.evaluate(() => (window as any).__installerTest.emit({ kind: 'finished', ok: false, cancelled: false, error: 'Checksum mismatch' }));
  await expect(page.getByText('Installation failed: Checksum mismatch')).toBeVisible();
});

test('self update can be postponed and restarted from its banner', async ({ page }) => {
  await host(page, { selfUpdate: true });
  await page.getByRole('button', { name: 'Not now', exact: true }).click();
  await page.locator('.mi-self-update').getByRole('button', { name: 'Update', exact: true }).click();
  await page.locator('.mi-self-update-dialog').getByRole('button', { name: 'Update', exact: true }).click();
  await expect.poll(() => page.evaluate(() => (window as any).__installerTest.calls.some((c: any) => c.command === 'selfUpdate'))).toBe(true);
  await page.evaluate(() => (window as any).__installerTest.emit({ kind: 'finished', ok: true, cancelled: false, error: null }, 'meridian.installer'));
  await expect.poll(() => page.evaluate(() => (window as any).__installerTest.calls.some((c: any) => c.command === 'launch' && c.args === 'meridian.installer'))).toBe(true);
});
