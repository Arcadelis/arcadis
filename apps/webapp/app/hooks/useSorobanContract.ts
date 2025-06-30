"use client"

import { useState, useCallback, useMemo } from "react"
import { useWallet } from "./useWallet"
import { toast } from "sonner"

export interface SorobanContractState {
  loading: boolean
  error: string | null
  success: boolean
}

export interface SorobanContractOptions {
  showSuccessToast?: boolean
  showErrorToast?: boolean
  successMessage?: string
  errorMessage?: string
}

export interface UseSorobanContractReturn<T> {
  // State
  state: SorobanContractState

  // Contract instance
  contract: T | null

  // Wallet info
  isConnected: boolean
  walletAddress: string | null

  // Contract execution
  executeContract: <R>(
    contractMethod: () => Promise<{ signAndSend: () => Promise<{ result: R }> }>,
    options?: SorobanContractOptions,
  ) => Promise<R | null>

  // Read-only contract calls
  readContract: <R>(contractMethod: () => Promise<R>, defaultValue: R) => Promise<R>

  // State management
  updateState: (updates: Partial<SorobanContractState>) => void
  resetState: () => void
}

export function useSorobanContract<T>(contractFactory: () => T | null): UseSorobanContractReturn<T> {
  const [state, setState] = useState<SorobanContractState>({
    loading: false,
    error: null,
    success: false,
  })

  const { isConnected, walletAddress, signTransaction } = useWallet()

  // Create contract instance
  const contract = useMemo(() => {
    try {
      return contractFactory()
    } catch (error) {
      console.error("Failed to create contract instance:", error)
      return null
    }
  }, [contractFactory])

  // Helper to update state
  const updateState = useCallback((updates: Partial<SorobanContractState>) => {
    setState((prev) => ({ ...prev, ...updates }))
  }, [])

  // Helper to reset state
  const resetState = useCallback(() => {
    setState({
      loading: false,
      error: null,
      success: false,
    })
  }, [])

  // Helper to handle errors
  const handleError = useCallback(
    (error: unknown, customMessage?: string) => {
      const errorMessage = error instanceof Error ? error.message : customMessage || "An error occurred"
      updateState({ error: errorMessage, loading: false, success: false })
      return null
    },
    [updateState],
  )

  // Configure contract for signing
  const configureContractSigning = useCallback(
    (contractInstance: any) => {
      if (!contractInstance?.client || !walletAddress) return false

      contractInstance.client.options.publicKey = walletAddress
      contractInstance.client.options.signTransaction = async (xdr: string) => {
        console.log("Contract client calling signTransaction with XDR:", xdr)

        try {
          const signResult = await signTransaction(xdr)
          console.log("Wallet sign result:", signResult)

          if (!signResult.success) {
            throw new Error(signResult.error || "Failed to sign transaction")
          }

          return {
            signedTxXdr: signResult.signedTxXdr!,
            signerAddress: walletAddress,
          }
        } catch (error) {
          console.error("Error in signTransaction:", error)
          throw error
        }
      }

      return true
    },
    [walletAddress, signTransaction],
  )

  // Execute a contract method that requires signing
  const executeContract = useCallback(
    async (
      contractMethod: () => Promise<{ signAndSend: () => Promise<any> }>,
      options: SorobanContractOptions = {},
    ) => {
      const {
        showSuccessToast = true,
        showErrorToast = true,
        successMessage = "Transaction completed successfully!",
        errorMessage = "Transaction failed",
      } = options

      if (!isConnected || !walletAddress || !contract) {
        const error = "Wallet not connected or contract not available"
        if (showErrorToast) toast.error(error)
        return handleError(new Error(error))
      }

      // Configure contract for signing
      if (!configureContractSigning(contract)) {
        const error = "Failed to configure contract signing"
        if (showErrorToast) toast.error(error)
        return handleError(new Error(error))
      }

      updateState({ loading: true, error: null, success: false })

      try {
        console.log("Executing contract method...")
        const tx = await contractMethod()
        console.log("Transaction created:", tx)

        console.log("Signing and sending transaction...")
        const { result } = await tx.signAndSend()
        console.log("Transaction result:", result)

        if (result !== undefined) {
          updateState({ loading: false, success: true })
          if (showSuccessToast) toast.success(successMessage)
          return result
        } else {
          throw new Error("Transaction failed - no result returned")
        }
      } catch (error) {
        console.error("Error in executeContract:", error)
        if (showErrorToast) toast.error(errorMessage)
        return handleError(error, errorMessage)
      }
    },
    [isConnected, walletAddress, contract, configureContractSigning, updateState, handleError],
  )

  // Execute a read-only contract method
  const readContract = useCallback(
    async (contractMethod: () => Promise<any>, defaultValue: any): Promise<any> => {
      if (!contract) return defaultValue

      try {
        return await contractMethod()
      } catch (error) {
        console.error("Error in readContract:", error)
        return defaultValue
      }
    },
    [contract],
  )

  return {
    state,
    contract,
    isConnected,
    walletAddress,
    executeContract,
    readContract,
    updateState,
    resetState,
  }
}
