import React from 'react'
import { useNavigate } from 'react-router-dom'
import { Home } from 'lucide-react'

const NotFound: React.FC = () => {
  const navigate = useNavigate()

  return (
    <div className="text-center py-16">
      <h1 className="text-8xl font-bold text-primary-500 mb-4">
        404
      </h1>
      <h2 className="text-2xl font-semibold text-white mb-6">
        Page Not Found
      </h2>
      <p className="text-warm-400 mb-8">
        The page you're looking for doesn't exist.
      </p>
      <button
        onClick={() => navigate('/dashboard')}
        className="btn-primary flex items-center gap-2 mx-auto"
      >
        <Home size={18} />
        Go to Dashboard
      </button>
    </div>
  )
}

export default NotFound
