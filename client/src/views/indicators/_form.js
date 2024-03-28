import React, { useState } from 'react'
import {
  CCol,
  CButton,
  CForm,
  CFormInput,
  CFormLabel,
  CRow,
} from '@coreui/react'

export default ({ handleSave, updateAttr, data, disable }) => {
  return (
    <CRow>
      <CCol xs={12}>
        <CForm>
          <div className="mb-3">
            <CFormLabel htmlFor="exampleFormControlInput1">Name</CFormLabel>
            <CFormInput
              type="text"
              id="exampleFormControlInput1"
              placeholder="Name1"
              disabled={disable}
              value={data.name || ""}
              onChange={(event) => updateAttr('name', event.target.value)}
            />
          </div>

          <div className="mb-3">
            <CFormLabel htmlFor="exampleFormControlInput2">Symbol</CFormLabel>
            <CFormInput
              type="text"
              id="exampleFormControlInput2"
              placeholder="Symbol1"
              disabled={disable}
              value={data.symbol || ""}
              onChange={(event) => updateAttr('symbol', event.target.value)}
            />
          </div>

          <div className="mb-3">
            <CFormLabel htmlFor="exampleFormControlInput2">Description</CFormLabel>
            <CFormInput
              type="text"
              id="exampleFormControlInput2"
              placeholder="Description1"
              disabled={disable}
              value={data.description || ""}
              onChange={(event) => updateAttr('description', event.target.value)}
            />
          </div>

          <div className="mb-3">
            <CFormLabel htmlFor="exampleFormControlInput2">IndicatorType</CFormLabel>
            <CFormInput
              type="text"
              id="exampleFormControlInput2"
              placeholder="IndicatorType1"
              disabled={disable}
              value={data.indicator_type || ""}
              onChange={(event) => updateAttr('indicator_type', event.target.value)}
            />
          </div>

          { !disable && <CButton
            color="info"
            variant="outline"
            onClick={() => handleSave()}
          >Create</CButton>}
        </CForm>
      </CCol>
    </CRow>
  )
}

