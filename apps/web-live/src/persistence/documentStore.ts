/**
 * Execution 03, Phase 14, Task 114: the browser storage adapter --
 * IndexedDB, not `localStorage` (a real document's JSON can grow past
 * `localStorage`'s ~5MB synchronous-API ceiling; IndexedDB is the
 * standard answer, and its async API matches this module's shape).
 * Stores exactly one blob: the real `CraftLoopSession.toJson()`
 * output, keyed by a fixed id -- Task 115's "simple document
 * management only, no full production library" means one current
 * document, not a library of named files.
 *
 * Deliberately untested by Vitest: jsdom has no real IndexedDB
 * implementation, and a fake would only prove this module agrees with
 * itself, not with a real browser. Task 117's "reload acceptance" is
 * proven by a real Playwright test instead
 * (`tests/e2e/persistence.spec.ts`), which exercises this exact code
 * against Chromium's real IndexedDB.
 */

const DB_NAME = 'craftloop-web-live'
const DB_VERSION = 1
const STORE_NAME = 'documents'
const DOCUMENT_KEY = 'current'

function openDb(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(DB_NAME, DB_VERSION)
    request.onupgradeneeded = () => {
      if (!request.result.objectStoreNames.contains(STORE_NAME)) {
        request.result.createObjectStore(STORE_NAME)
      }
    }
    request.onsuccess = () => resolve(request.result)
    request.onerror = () => reject(request.error as DOMException)
  })
}

async function withStore<T>(mode: IDBTransactionMode, fn: (store: IDBObjectStore) => IDBRequest<T>): Promise<T> {
  const db = await openDb()
  try {
    return await new Promise<T>((resolve, reject) => {
      const tx = db.transaction(STORE_NAME, mode)
      const request = fn(tx.objectStore(STORE_NAME))
      request.onsuccess = () => resolve(request.result)
      request.onerror = () => reject(request.error as DOMException)
    })
  } finally {
    db.close()
  }
}

/** Persists the real document JSON (`CraftLoopSession.toJson()`'s output) -- never any UI-only/ephemeral state (Task 118). */
export async function saveDocumentJson(json: string): Promise<void> {
  await withStore('readwrite', (store) => store.put(json, DOCUMENT_KEY))
}

/** The last saved document JSON, or `null` if nothing has been saved yet. */
export async function loadDocumentJson(): Promise<string | null> {
  const result = await withStore<string | undefined>('readonly', (store) => store.get(DOCUMENT_KEY))
  return result ?? null
}
