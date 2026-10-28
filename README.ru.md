<div align="center">

<img src="src-tauri/icons/128x128.png" alt="Meridian Installer" width="96">

# Meridian Installer

### Устанавливает SDK и инструменты Meridian отдельными компонентами с версиями

[![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](LICENSE)
[![Rust 1.99](https://img.shields.io/badge/Rust-1.99-orange.svg)](rust-toolchain.toml)
[![Tauri 2](https://img.shields.io/badge/Tauri-2-24C8DB.svg)](https://tauri.app/)

[English](README.md) | Русский

</div>

---

## Обзор

**Meridian Installer** — настольное приложение в духе Visual Studio Installer, которое
устанавливает продукты и инструменты [Meridian Engine](https://github.com/F000NKKK/Meridian-Engine).
Вкладки «Установлено» и «Доступно», карточки продуктов с кнопками «Изменить», «Обновить» и
«Удалить», рабочие нагрузки («Разработка игр», «Разработка движка»), отдельные компоненты и
ход установки с паузой и отменой. Приложение построено на установочных крейтах
[Substrate Platform](https://github.com/F000NKKK/Substrate-Platform); здесь лежат каталог,
домашняя папка Meridian и окно.

| Часть | Где | Что дает |
|---|---|---|
| `meridian-installer-core` | [`crates/meridian-installer-core`](crates/meridian-installer-core) | Корень композиции: домашняя папка, определение системы, токен GitHub, адаптеры, каталог, сеансы и операции. |
| `meridian-installer` (CLI) | [`crates/meridian-installer-cli`](crates/meridian-installer-cli) | Консольные `list`, `plan`, `install`, `update`, `remove`, `status`. |
| Приложение | [`src-tauri`](src-tauri), [`src`](src) | Серверная часть на Tauri 2 и окно на React, английский и русский. |
| Каталог | [`catalog/catalog.sdata`](catalog/catalog.sdata) | Что можно установить: установщик, Meridian IDLE, CLI, SDK, примеры, ресурсы, Rust. |

## Что устанавливается

- **Meridian IDLE** — среда разработки (продукт).
- **Компоненты**: `meridian` CLI, SDK движка (выпуск по тегу, исходный код), примеры и образцы
  ресурсов, набор инструментов Rust и его целевые платформы.
- **Рабочие нагрузки** выбирают компоненты под задачу: «Разработка игр» и «Разработка движка».

Все лежит в `~/.meridian` (переменная `MERIDIAN_HOME` переносит папку). Лимит запросов GitHub
снимает `GITHUB_TOKEN` или выполненный вход в `gh`. Что должен публиковать выпуск каждого
репозитория — в [контракте выпусков](docs/release-contract.md).

## Разработка

Склонируйте этот репозиторий и [Substrate-Platform](https://github.com/F000NKKK/Substrate-Platform)
рядом.

```bash
(cd ../Substrate-Platform/ui && npm ci)
npm install
npm run typecheck && npm test
cargo test --workspace --exclude meridian-installer
cargo run -p meridian-installer-cli -- list
npm run tauri dev
```

См. [CONTRIBUTING.md](CONTRIBUTING.md), [AGENTS.md](AGENTS.md) и [ADR](docs/adr).

## Состояние

Ранняя версия. Ядро и CLI проверены целиком на заменителях; приложению нужен первый реальный
запуск. Не сделано: размещение самого установщика и продолжение прерванной установки
(см. TODO в [контракте выпусков](docs/release-contract.md)).

## Лицензия

[AGPL-3.0](LICENSE).
