import React, { useEffect } from 'react'
import { useSelector, useDispatch } from 'react-redux'

import { NavLink } from 'react-router-dom'
import {
  CButton,
  CButtonGroup,
  CNavLink
} from '@coreui/react'

import TickerSlice from '../../slices/TickerSlice'

import Form from './_form'

import { useParams } from 'react-router-dom'

export default () => {
  const params = useParams()
  const dispatch = useDispatch()

  const obj = useSelector(state => state.tickers.obj);

  useEffect(() => {
    dispatch(TickerSlice.actions.show(params.id))
  }, [])

  return (
    <React.Fragment>
      <Form disable={true}  data={obj}/>

      <CButtonGroup role="group" aria-label="Row Actions">
          <CNavLink to={`/currencies`} as={NavLink}>
            <CButton color="info" variant="outline">Return to List</CButton>
          </CNavLink>
          <CNavLink to={`/currencies/${obj.id}/edit`} as={NavLink}>
            <CButton color="primary" variant="outline">Edit</CButton>
          </CNavLink>
          <CButton
            color="danger"
            variant="outline"
            onClick={() => handleDelete(obj.id)}
          >
            Delete
          </CButton>
      </CButtonGroup>
    </React.Fragment>
  )
}

