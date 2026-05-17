import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'

const HARDCODED_ADDRESS = 'tb1qd28npep0s8frcm3y7dxqajkcy2m40eysplyr9v'

interface AddressData {
  confirmed_sats: number
  unconfirmed_sats: number
}

function App() {
  const [data, setData] = useState<AddressData | null>(null)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    invoke<AddressData>('sync_address', { address: HARDCODED_ADDRESS })
      .then(setData)
      .catch((e) => setError(String(e)))
  }, [])

  return (
    <div style={{ fontFamily: 'monospace', padding: '2rem' }}>
      <h1>BTC Wallet — Walking Skeleton</h1>
      <p>Address: {HARDCODED_ADDRESS}</p>
      {error && <p style={{ color: 'red' }}>Error: {error}</p>}
      {!data && !error && <p>Syncing…</p>}
      {data && (
        <ul>
          <li>Confirmed: {data.confirmed_sats} sats</li>
          <li>Unconfirmed: {data.unconfirmed_sats} sats</li>
        </ul>
      )}
    </div>
  )
}

export default App
