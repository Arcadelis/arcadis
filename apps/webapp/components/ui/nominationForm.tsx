'use client'

import { Send } from 'lucide-react'
import type * as React from 'react'
import { useState } from 'react'
import { Button } from './button'
import { Input } from './input'

interface GameNominationFormProps {
	formTitle?: string
	submitButtonText?: string
	placeholderText?: string
	successMessage?: string
	formId?: string
	requiredFields?: string[]
	labelText?: string
	validationRules?: { maxLength?: number }
	maxLength?: number
	buttonColor?: string
	buttonIcon?: string | null
	inputBorderRadius?: string
	backgroundColor?: string
}

const GameNominationForm: React.FC<GameNominationFormProps> = ({
	formTitle = 'Submit Your Nomination',
	submitButtonText = 'Submit Nomination',
	placeholderText = 'Enter the game name',
	successMessage = 'Thank you for your nomination!*',
	formId = 'game-nomination-form',
	requiredFields = ['gameName'],
	labelText = 'Game Name',
	validationRules = {},
	maxLength = 50,
	buttonColor = 'bg-blue-600 hover:bg-blue-700',
	buttonIcon = null,
	inputBorderRadius = 'rounded-3xl',
	backgroundColor = 'bg-[#111827]',
}) => {
	const [gameName, setGameName] = useState('')
	const [submitted, setSubmitted] = useState(false)
	const [error, setError] = useState('')

	const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
		e.preventDefault()
		setError('')

		if (requiredFields.includes('gameName') && !gameName.trim()) {
			setError('Game name is required*')
			return
		}

		if (validationRules.maxLength && gameName.length > validationRules.maxLength) {
			setError(`Game name cannot exceed ${validationRules.maxLength} characters`)
			return
		}

		setSubmitted(true)
	}

	return (
		<div
			className={`${backgroundColor} p-6 ${inputBorderRadius} shadow-xl w-full max-w-[842.67px]`}
		>
			<h5 className="font-semibold text-[18.91px] text-white mb-5">{formTitle}</h5>
			{!submitted ? (
				<form id={formId} onSubmit={handleSubmit} className="flex flex-col">
					<label htmlFor={formId} className="font-medium text-[13.78px] text-[#9CA3AF] mb-1">
						{labelText}
					</label>
					<Input
						type="text"
						placeholder={placeholderText}
						className="bg-white/5 border-white/10"
						value={gameName}
						onChange={(e) => setGameName(e.target.value)}
						maxLength={maxLength}
					/>
					{error && <p className="text-red-500 text-xs mt-1">{error}</p>}
					<Button type="submit" className={`${buttonColor} font-semibold mt-4`}>
						{buttonIcon ? <img src={buttonIcon} alt="Icon" className="w-4" /> : <Send />}
						{submitButtonText}
					</Button>
				</form>
			) : (
				<p className="text-green-500 text-xs mt-1">{successMessage}</p>
			)}
		</div>
	)
}

export default GameNominationForm
