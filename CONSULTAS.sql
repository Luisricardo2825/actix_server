SELECT 
    CONTRATO.NUMCONTRATO,
    CONTRATO.PARCELAS_TOTAIS,
    NVL(FATURADAS.PARCELAS_FATURADAS, 0) AS PARCELAS_FATURADAS,
    CONTRATO.PARCELAS_TOTAIS - NVL(FATURADAS.PARCELAS_FATURADAS, 0) AS PARCELAS_PENDENTES
FROM (
    -- Cálculo das parcelas totais para todos os contratos
    SELECT 
        con.NUMCONTRATO,
        CASE
            WHEN con.TIPO = 'A' THEN con.PARCELAQTD / 12
            WHEN con.TIPO = 'L' THEN con.PARCELAQTD / 1
            WHEN con.TIPO = 'M' THEN con.PARCELAQTD / 1
            WHEN con.TIPO = 'B' THEN con.PARCELAQTD / 2
            WHEN con.TIPO = 'T' THEN con.PARCELAQTD / 3
            WHEN con.TIPO = 'Q' THEN con.PARCELAQTD / 4
            WHEN con.TIPO = 'S' THEN con.PARCELAQTD / 6
        END AS PARCELAS_TOTAIS
    FROM TCSCON con
) CONTRATO
LEFT JOIN (
    -- Cálculo das parcelas faturadas para todos os contratos
    SELECT 
        A.NUMCONTRATO,
        COUNT(*) AS PARCELAS_FATURADAS
    FROM (
        SELECT 
            TGFCAB.NUMCONTRATO,
            TGFCAB.DTVAL,
            (SELECT CASE
                        WHEN con.TIPO = 'A' THEN con.PARCELAQTD / 12
                        WHEN con.TIPO = 'L' THEN con.PARCELAQTD / 1
                        WHEN con.TIPO = 'M' THEN con.PARCELAQTD / 1
                        WHEN con.TIPO = 'B' THEN con.PARCELAQTD / 2
                        WHEN con.TIPO = 'T' THEN con.PARCELAQTD / 3
                        WHEN con.TIPO = 'Q' THEN con.PARCELAQTD / 4
                        WHEN con.TIPO = 'S' THEN con.PARCELAQTD / 6
                    END
             FROM TCSCON con
             WHERE con.NUMCONTRATO = TGFCAB.NUMCONTRATO
            ) AS PARCELAS_TOTAIS
        FROM TGFCAB
        WHERE TGFCAB.TIPMOV = 'V'
        GROUP BY TGFCAB.DTVAL, TGFCAB.NUMCONTRATO
    ) A
    GROUP BY A.NUMCONTRATO
) FATURADAS ON CONTRATO.NUMCONTRATO = FATURADAS.NUMCONTRATO;