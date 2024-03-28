import React, {useEffect, useState} from 'react'
import { NavLink } from 'react-router-dom'
import {
  CButton,
  CButtonGroup,
  CAlert,
  CCard,
  CCardBody,
  CNavLink,
  CCol,
  CRow,
  CTable,
  CTableBody,
  CTableDataCell,
  CTableHead,
  CTableHeaderCell,
  CTableRow,
} from '@coreui/react'

import { useDispatch, useSelector } from 'react-redux'

import TheoryPortfolioSlice from '../../slices/TheoryPortfolioSlice'

const Tables = () => {
  const [messageDanger, setMessageDanger] = useState(null)
  const list = useSelector(state => state.theory_portfolios.list);
  const dispatch = useDispatch()

  useEffect(() => {
    dispatch(TheoryPortfolioSlice.actions.index());
  },[])

  const handleDelete = (id) => {
    dispatch(TheoryPortfolioSlice.actions.destroy(id))
      .then(() => dispatch(TheoryPortfolioSlice.actions.index()))
      .then(() => setMessageDanger(`TheoryPortfolio ${id} Deletado!`));
  }

  return (
    <CRow>
      <CCol xs={12}>
        <CCard className="mb-4">
          <CCardBody>
            <CNavLink to='/theory_portfolios/create' as={NavLink}>
              <CButton color="info" variant="outline">Create</CButton>
            </CNavLink>
          </CCardBody>
        </CCard>
        {messageDanger && <CAlert color="danger"> {messageDanger} </CAlert>}
        <CCard className="mb-4">
          <CCardBody>
              <CTable>
                <CTableHead>
                  <CTableRow>
                    <CTableHeaderCell scope="col">#</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Ticker_id</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Date</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Close</CTableHeaderCell>
                    <CTableHeaderCell scope="col">DailyFactor</CTableHeaderCell>
                    <CTableHeaderCell scope="col">AcumulatorFactor</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Actions</CTableHeaderCell>
                  </CTableRow>
                </CTableHead>
                <CTableBody>
                  {list.map((elem, idx) => {
                    return (
                      <CTableRow key={idx} >
                        <CTableDataCell>{elem.id}</CTableDataCell>
                        <CTableDataCell>
                          <CNavLink to={`/tickers/${elem.ticker_id}`} as={NavLink}>
                            #{elem.ticker_id}
                          </CNavLink>
                        </CTableDataCell>
                        <CTableDataCell>{elem.date}</CTableDataCell>
                        <CTableDataCell>{elem.close}</CTableDataCell>
                        <CTableDataCell>{elem.daily_factor}</CTableDataCell>
                        <CTableDataCell>{elem.accumulated_factor}</CTableDataCell>
                        <CTableDataCell>
                          <CButtonGroup role="group" aria-label="Row Actions">
                              <CNavLink to={`/theory_portfolios/${elem.id}`} as={NavLink}>
                                <CButton color="info" variant="outline">Show</CButton>
                              </CNavLink>
                              <CNavLink to={`/theory_portfolios/${elem.id}/edit`} as={NavLink}>
                                <CButton color="primary" variant="outline">Edit</CButton>
                              </CNavLink>
                              <CButton
                                color="danger"
                                variant="outline"
                                onClick={() => handleDelete(elem.id)}
                              >
                                Delete
                              </CButton>
                          </CButtonGroup>
                        </CTableDataCell>
                      </CTableRow>
                    )
                  })}
                </CTableBody>
              </CTable>
          </CCardBody>
        </CCard>
      </CCol>
    </CRow>
  )
}

export default Tables
