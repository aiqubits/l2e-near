export const Form = ({ onSubmit, currentAccountId }) => {
    return (
      <form onSubmit={onSubmit} className="p-4 border rounded shadow-sm">
          <h2 className="mb-4">Guest Book</h2>
          <p>Sign the guest book, {currentAccountId}!</p>
          <div className="mb-3">
            <input
              autoComplete="off"
              autoFocus
              id="message"
              className="form-control"
              required
            />
          </div>
          <div className="mb-3">
            <div className="input-group">
              <input
                autoComplete="off"
                defaultValue={"0"}
                id="donation"
                min="0"
                step="0.01"
                type="number"
                className="form-control"
              />
            </div>
          </div>
          <button type="submit" className="btn btn-primary w-100">
            Sign Guest Book
          </button>
      </form>
    );
  }
