import axios from 'axios'

const withAuthentication = axios.create({
  baseURL: 'http://localhost:8000',
  timeout: 180 * 1000
});

export default withAuthentication
